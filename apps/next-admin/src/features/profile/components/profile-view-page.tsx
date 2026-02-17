'use client';

import PageContainer from '@/components/layout/page-container';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useCurrentUser } from '@/store/auth-store';

export default function ProfileViewPage() {
  const user = useCurrentUser();

  return (
    <PageContainer>
      <div className='flex w-full flex-col gap-6 p-4'>
        <h1 className='text-2xl font-bold tracking-tight'>Profile</h1>
        {user ? (
          <div className='grid gap-4 md:grid-cols-2'>
            <Card>
              <CardHeader>
                <CardTitle>Account Information</CardTitle>
              </CardHeader>
              <CardContent className='space-y-3'>
                <div>
                  <p className='text-muted-foreground text-xs font-medium uppercase tracking-wider'>Name</p>
                  <p className='text-sm font-medium'>{user.name || '—'}</p>
                </div>
                <div>
                  <p className='text-muted-foreground text-xs font-medium uppercase tracking-wider'>Email</p>
                  <p className='text-sm font-medium'>{user.email}</p>
                </div>
                <div>
                  <p className='text-muted-foreground text-xs font-medium uppercase tracking-wider'>Role</p>
                  <p className='text-sm font-medium'>{user.role}</p>
                </div>
                <div>
                  <p className='text-muted-foreground text-xs font-medium uppercase tracking-wider'>Status</p>
                  <p className='text-sm font-medium'>{user.status}</p>
                </div>
                <div>
                  <p className='text-muted-foreground text-xs font-medium uppercase tracking-wider'>Workspace</p>
                  <p className='text-sm font-medium'>{user.tenantSlug || '—'}</p>
                </div>
                <div>
                  <p className='text-muted-foreground text-xs font-medium uppercase tracking-wider'>Member since</p>
                  <p className='text-sm font-medium'>{new Date(user.createdAt).toLocaleDateString()}</p>
                </div>
              </CardContent>
            </Card>
          </div>
        ) : (
          <p className='text-muted-foreground text-sm'>Loading profile...</p>
        )}
      </div>
    </PageContainer>
  );
}
