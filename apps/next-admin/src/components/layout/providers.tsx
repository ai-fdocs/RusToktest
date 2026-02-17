'use client';
import React, { useEffect } from 'react';
import { ActiveThemeProvider } from '../themes/active-theme';
import { useAuthStore } from '@/store/auth-store';

function AuthInitializer() {
  const loadCurrentUser = useAuthStore((s) => s.loadCurrentUser);
  const token = useAuthStore((s) => s.token);

  useEffect(() => {
    if (token) {
      loadCurrentUser();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return null;
}

export default function Providers({
  activeThemeValue,
  children
}: {
  activeThemeValue: string;
  children: React.ReactNode;
}) {
  return (
    <ActiveThemeProvider initialTheme={activeThemeValue}>
      <AuthInitializer />
      {children}
    </ActiveThemeProvider>
  );
}
