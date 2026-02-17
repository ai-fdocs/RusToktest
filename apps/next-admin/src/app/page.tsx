import { redirect } from 'next/navigation';
import { cookies } from 'next/headers';

export default async function Page() {
  const cookieStore = await cookies();
  const token = cookieStore.get('auth_token')?.value;

  if (token) {
    redirect('/dashboard/overview');
  } else {
    redirect('/auth/sign-in');
  }
}
