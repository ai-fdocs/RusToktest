"use client";

import { useEffect } from "react";

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
}

function readCookie(name: string) {
  return document.cookie
    .split("; ")
    .find((cookie) => cookie.startsWith(`${name}=`))
    ?.split("=")[1];
}

function persistLocaleCookie(locale: string) {
  document.cookie = `${NEXT_LOCALE_COOKIE}=${locale}; path=/; max-age=31536000`;
}

export default function LocaleSync({ locale }: LocaleSyncProps) {
  useEffect(() => {
    if (typeof window === "undefined") {
      return;
    }

    persistLocale(locale);
    if (readCookie(NEXT_LOCALE_COOKIE) !== locale) {
      persistLocaleCookie(locale);
    }
  }, [locale]);

  return null;
}
