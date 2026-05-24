#!/usr/bin/env node

import { readFileSync, existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, "../..");

const requiredDocs = [
  "docs/research/dioxus-ffa-ui-migration-plan.md",
  "docs/research/dioxus-ffa-pilot-connectivity-map.md",
  "docs/verification/ffa-ui-parity-checklist.md",
];

const requiredPlanHeadings = [
  "Фазы реализации",
  "Принцип исполнения backlog (одна задача за итерацию)",
  "Политика актуализации verification scripts",
  "Phase-gate критерии (обязательные переходы между фазами)",
  "KPI parity (измеримые пороги)",
  "RACI (кто принимает phase-gates)",
];

const requiredChecklistPatterns = [
  /- \[[ xX]\] Native path \(Leptos SSR\/hydrate\) работает для целевого сценария\./,
  /- \[[ xX]\] GraphQL fallback работает для того же сценария\./,
  /- \[[ xX]\] UI слой не владеет transport\/business логикой\./,
  /- \[[ xX]\] Доступ к transport идёт через core ports\./,
  /- \[[ xX]\] Core слой не зависит от `leptos\*`\./,
  /- \[[ xX]\] Выполнен `npm run verify:ffa:ui:migration`\./,
];

const requiredConnectivityMentions = ["rustok-pages", "rustok-search"];

function assertFileExists(relPath) {
  const fullPath = path.join(repoRoot, relPath);
  if (!existsSync(fullPath)) {
    throw new Error(`Отсутствует обязательный документ: ${relPath}`);
  }
  return fullPath;
}

function normalizeMarkdown(content) {
  return content.replace(/\r\n/g, "\n").replace(/[ \t]+$/gm, "");
}

function getMarkdownHeadings(content) {
  return content
    .split("\n")
    .map((line) => line.match(/^#{1,6}\s+(.*)$/)?.[1]?.trim())
    .filter(Boolean);
}

function readRequiredDocs() {
  const [planPath, connectivityPath, checklistPath] = requiredDocs.map(assertFileExists);

  return {
    plan: normalizeMarkdown(readFileSync(planPath, "utf8")),
    connectivity: normalizeMarkdown(readFileSync(connectivityPath, "utf8")),
    checklist: normalizeMarkdown(readFileSync(checklistPath, "utf8")),
  };
}

function collectValidationErrors({ plan, connectivity, checklist }) {
  const errors = [];

  const planHeadings = new Set(getMarkdownHeadings(plan));
  requiredPlanHeadings.forEach((heading) => {
    if (!planHeadings.has(heading)) {
      errors.push(`Не найден обязательный heading в migration plan: ${heading}`);
    }
  });

  requiredChecklistPatterns.forEach((pattern) => {
    if (!pattern.test(checklist)) {
      errors.push(`Не найден обязательный checklist-паттерн: ${pattern}`);
    }
  });

  requiredConnectivityMentions.forEach((mention) => {
    if (!connectivity.includes(mention)) {
      errors.push(`Не найден обязательный пилот в connectivity map: ${mention}`);
    }
  });

  return errors;
}

try {
  const docs = readRequiredDocs();
  const errors = collectValidationErrors(docs);

  if (errors.length > 0) {
    console.error("[verify-ffa-ui-migration-contract] FAIL");
    errors.forEach((error) => console.error(`- ${error}`));
    process.exit(1);
  }

  console.log("[verify-ffa-ui-migration-contract] PASS");
  console.log("Проверены обязательные документы и baseline-контракты FFA migration.");
} catch (error) {
  console.error("[verify-ffa-ui-migration-contract] FAIL");
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
