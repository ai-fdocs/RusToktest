import type { ReactNode } from "react";
import { setRequestLocale } from "next-intl/server";

import LocaleSync from "@/components/locale-sync";

export default function LocaleLayout({
  children,
  params: { locale },
}: {
  children: ReactNode;
  params: { locale: string };
}) {
  setRequestLocale(locale);

  return (
    <>
      <LocaleSync locale={locale} />
      {children}
    </>
  );
}
