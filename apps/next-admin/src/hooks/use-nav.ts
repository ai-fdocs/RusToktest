'use client';

import { useMemo } from 'react';
import { useCurrentUser } from '@/store/auth-store';
import type { NavItem } from '@/types';

/**
 * Hook to filter navigation items based on user role.
 * Replaces the Clerk-based implementation with our own auth store.
 */
export function useFilteredNavItems(items: NavItem[]) {
  const user = useCurrentUser();

  const filteredItems = useMemo(() => {
    return items
      .filter((item) => {
        // No access restrictions
        if (!item.access) return true;

        // Items requiring org context are hidden (no orgs in our system)
        if (item.access.requireOrg) return false;

        // Role-based filtering
        if (item.access.role && user?.role?.toLowerCase() !== item.access.role?.toLowerCase()) {
          return false;
        }

        return true;
      })
      .map((item) => {
        if (item.items && item.items.length > 0) {
          return {
            ...item,
            items: item.items.filter((child) => {
              if (!child.access) return true;
              if (child.access.requireOrg) return false;
              if (child.access.role && user?.role?.toLowerCase() !== child.access.role?.toLowerCase()) {
                return false;
              }
              return true;
            })
          };
        }
        return item;
      });
  }, [items, user?.role]);

  return filteredItems;
}
