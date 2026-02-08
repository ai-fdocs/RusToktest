import type { ReactNode } from "react";
import { setRequestLocale } from "next-intl/server";

export default function LocaleLayout({
  children,
  params: { locale },
}: {
  children: ReactNode;
  params: { locale: string };
}) {
  setRequestLocale(locale);

  return children;
}
