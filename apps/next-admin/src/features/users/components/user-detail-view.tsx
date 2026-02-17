'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { graphqlRequest } from '@/lib/graphql';
import { useAuthStore } from '@/store/auth-store';
import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import Link from 'next/link';
import { Button } from '@/components/ui/button';

interface UserDetail {
  id: string;
  email: string;
  name: string | null;
  role: string;
  status: string;
  createdAt: string;
  tenantName: string | null;
}

interface UserDetailResponse {
  user: UserDetail | null;
}

const USER_QUERY = `
query User($id: UUID!) {
  user(id: $id) {
    id
    email
    name
    role
    status
    createdAt
    tenantName
  }
}
`;

export default function UserDetailView({ userId }: { userId: string }) {
  const token = useAuthStore((s) => s.token);
  const tenantSlug = useAuthStore((s) => s.tenantSlug);
  const [user, setUser] = useState<UserDetail | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetch = async () => {
      if (!token) return;
      try {
        const data = await graphqlRequest<{ id: string }, UserDetailResponse>(
          USER_QUERY,
          { id: userId },
          token,
          tenantSlug
        );
        setUser(data.user);
      } catch {
        toast.error('Failed to load user');
      } finally {
        setIsLoading(false);
      }
    };
    fetch();
  }, [userId, token, tenantSlug]);

  if (isLoading) {
    return <p className='text-muted-foreground text-sm'>Loading...</p>;
  }

  if (!user) {
    return (
      <p className='text-sm text-red-600'>User not found.</p>
    );
  }

  return (
    <div className='space-y-4'>
      <div>
        <Button variant='outline' size='sm' asChild>
          <Link href='/dashboard/users'>← Back to Users</Link>
        </Button>
      </div>
      <Card>
        <CardHeader>
          <CardTitle>{user.name || user.email}</CardTitle>
        </CardHeader>
        <CardContent className='grid gap-3 md:grid-cols-2 lg:grid-cols-3'>
          {[
            { label: 'Email', value: user.email },
            { label: 'Name', value: user.name || '—' },
            { label: 'Role', value: user.role },
            { label: 'Status', value: (
              <Badge variant={user.status === 'ACTIVE' ? 'default' : 'secondary'}>
                {user.status}
              </Badge>
            )},
            { label: 'Workspace', value: user.tenantName || '—' },
            { label: 'Member Since', value: new Date(user.createdAt).toLocaleDateString() },
            { label: 'ID', value: <span className='font-mono text-xs'>{user.id}</span> }
          ].map(({ label, value }) => (
            <div key={label}>
              <p className='text-muted-foreground text-xs font-medium uppercase tracking-wider'>{label}</p>
              <div className='mt-1 text-sm font-medium'>{value}</div>
            </div>
          ))}
        </CardContent>
      </Card>
    </div>
  );
}
