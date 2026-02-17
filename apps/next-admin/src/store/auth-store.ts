'use client';

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { AuthUser } from '@/lib/auth-api';
import { signIn, signUp, signOut, fetchCurrentUser } from '@/lib/auth-api';

interface AuthState {
  token: string | null;
  tenantSlug: string | null;
  user: AuthUser | null;
  isLoading: boolean;

  // Actions
  login: (email: string, password: string, tenantSlug: string) => Promise<void>;
  register: (email: string, password: string, tenantSlug: string, name?: string) => Promise<void>;
  logout: () => Promise<void>;
  loadCurrentUser: () => Promise<void>;
  setTokenAndTenant: (token: string, tenantSlug: string | null) => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      token: null,
      tenantSlug: null,
      user: null,
      isLoading: false,

      login: async (email, password, tenantSlug) => {
        set({ isLoading: true });
        try {
          const result = await signIn(email, password, tenantSlug);
          set({
            token: result.token,
            tenantSlug: result.user.tenantSlug ?? tenantSlug,
            user: result.user,
            isLoading: false
          });
          // Set cookie for middleware route protection
          document.cookie = `auth_token=${result.token}; path=/; max-age=${7 * 24 * 3600}; SameSite=Lax`;
        } catch (err) {
          set({ isLoading: false });
          throw err;
        }
      },

      register: async (email, password, tenantSlug, name) => {
        set({ isLoading: true });
        try {
          const result = await signUp(email, password, tenantSlug, name);
          set({
            token: result.token,
            tenantSlug: result.user.tenantSlug ?? tenantSlug,
            user: result.user,
            isLoading: false
          });
          document.cookie = `auth_token=${result.token}; path=/; max-age=${7 * 24 * 3600}; SameSite=Lax`;
        } catch (err) {
          set({ isLoading: false });
          throw err;
        }
      },

      logout: async () => {
        const { token, tenantSlug } = get();
        if (token) {
          await signOut(token, tenantSlug);
        }
        set({ token: null, tenantSlug: null, user: null });
        // Clear the auth cookie
        document.cookie = 'auth_token=; path=/; max-age=0';
      },

      loadCurrentUser: async () => {
        const { token, tenantSlug } = get();
        if (!token) return;
        set({ isLoading: true });
        const user = await fetchCurrentUser(token, tenantSlug);
        if (user) {
          set({ user, isLoading: false });
        } else {
          // Token invalid — clear everything
          set({ token: null, tenantSlug: null, user: null, isLoading: false });
          document.cookie = 'auth_token=; path=/; max-age=0';
        }
      },

      setTokenAndTenant: (token, tenantSlug) => {
        set({ token, tenantSlug });
        document.cookie = `auth_token=${token}; path=/; max-age=${7 * 24 * 3600}; SameSite=Lax`;
      }
    }),
    {
      name: 'rustok-auth',
      partialize: (state) => ({
        token: state.token,
        tenantSlug: state.tenantSlug,
        user: state.user
      })
    }
  )
);

// Convenience hooks
export const useCurrentUser = () => useAuthStore((s) => s.user);
export const useIsAuthenticated = () => useAuthStore((s) => !!s.token);
export const useAuthToken = () => useAuthStore((s) => s.token);
