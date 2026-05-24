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

const requiredPlanSections = [
  "## Фазы реализации",
  "## Принцип исполнения backlog (одна задача за итерацию)",
  "## Политика актуализации verification scripts",
  "## Phase-gate критерии (обязательные переходы между фазами)",
  "## KPI parity (измеримые пороги)",
  "## RACI (кто принимает phase-gates)",
];

const requiredChecklistPatterns = [
  /- \[ \] Native path \(Leptos SSR\/hydrate\) работает для целевого сценария\./,
  /- \[ \] GraphQL fallback работает для того же сценария\./,
  /- \[ \] UI слой не владеет transport\/business логикой\./,
  /- \[ \] Доступ к transport идёт через core ports\./,
  /- \[ \] Core слой не зависит от `leptos\*`\./,
  /- \[ \] Выполнен `npm run verify:ffa:ui:migration`\./,
];

function assertFileExists(relPath) {
  const fullPath = path.join(repoRoot, relPath);
  if (!existsSync(fullPath)) {
    throw new Error(`Отсутствует обязательный документ: ${relPath}`);
  }
  return fullPath;
}

function assertContains(content, value, label) {
  if (!content.includes(value)) {
    throw new Error(`Не найден обязательный фрагмент (${label}): ${value}`);
  }
}

function assertMatches(content, pattern, label) {
  if (!pattern.test(content)) {
    throw new Error(`Не найден обязательный паттерн (${label}): ${pattern}`);
  }
}

function normalizeMarkdown(content) {
  return content.replace(/\r\n/g, "\n").replace(/[ \t]+$/gm, "");
}

try {
  const planPath = assertFileExists(requiredDocs[0]);
  const connectivityPath = assertFileExists(requiredDocs[1]);
  const checklistPath = assertFileExists(requiredDocs[2]);

  const plan = normalizeMarkdown(readFileSync(planPath, "utf8"));
  const checklist = normalizeMarkdown(readFileSync(checklistPath, "utf8"));
  const connectivity = normalizeMarkdown(readFileSync(connectivityPath, "utf8"));

  requiredPlanSections.forEach((section) => {
    assertContains(plan, section, "plan section");
  });

  requiredChecklistPatterns.forEach((pattern) => {
    assertMatches(checklist, pattern, "checklist item");
  });

  assertContains(
    connectivity,
    "rustok-pages",
    "pilot connectivity map should include rustok-pages",
  );
  assertContains(
    connectivity,
    "rustok-search",
    "pilot connectivity map should include rustok-search",
  );

  console.log("[verify-ffa-ui-migration-contract] PASS");
  console.log("Проверены обязательные документы и baseline-контракты FFA migration.");
} catch (error) {
  console.error("[verify-ffa-ui-migration-contract] FAIL");
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
