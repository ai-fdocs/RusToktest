"use client";

import { useEffect } from "react";
import { usePathname, useRouter } from "next/navigation";

import { locales } from "@/i18n";

const STORAGE_KEY = "rustok-admin-locale";
const NEXT_LOCALE_COOKIE = "NEXT_LOCALE";

type LocaleSyncProps = {
  locale: string;
};

function persistLocale(locale: string) {
  try {
    window.localStorage.setItem(STORAGE_KEY, locale);
  } catch {
    // Ignore storage errors (e.g. disabled storage).
  }

  document.cookie = `${NEXT_LOCALE_COOKIE}=${locale}; path=/; max-age=31536000`;
}

function readCookie(name: string) {
  return document.cookie
    .split("; ")
    .find((cookie) => cookie.startsWith(`${name}=`))
    ?.split("=")[1];
}

function replaceLocaleInPath(pathname: string, currentLocale: string, nextLocale: string) {
  const localePrefix = `/${currentLocale}`;
  if (pathname === localePrefix) {
    return `/${nextLocale}`;
  }

  if (pathname.startsWith(`${localePrefix}/`)) {
    return `/${nextLocale}${pathname.slice(localePrefix.length)}`;
  }

  return `/${nextLocale}${pathname}`;
}

export default function LocaleSync({ locale }: LocaleSyncProps) {
  const router = useRouter();
  const pathname = usePathname();

  useEffect(() => {
    if (typeof window === "undefined") {
      return;
    }

    const storedLocale = window.localStorage.getItem(STORAGE_KEY);
    const cookieLocale = readCookie(NEXT_LOCALE_COOKIE);
    const isStoredValid = storedLocale
      ? locales.includes(storedLocale as (typeof locales)[number])
      : false;

    if (!cookieLocale && storedLocale && isStoredValid && storedLocale !== locale) {
      persistLocale(storedLocale);
      const nextPath = replaceLocaleInPath(pathname, locale, storedLocale);
      router.replace(nextPath);
      return;
    }

    persistLocale(locale);
  }, [locale, pathname, router]);

  return null;
}
