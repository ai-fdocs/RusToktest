import { redirect } from 'next/navigation';
import { cookies } from 'next/headers';

export default async function Dashboard() {
  const cookieStore = await cookies();
  const token = cookieStore.get('auth_token')?.value;

  if (!token) {
    return redirect('/auth/sign-in');
  }

  redirect('/dashboard/overview');
}
