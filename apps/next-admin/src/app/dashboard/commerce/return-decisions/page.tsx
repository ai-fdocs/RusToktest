import { auth } from '@/auth';
import { ReturnDecisionsTemplate } from '@rustok/commerce-admin';

export const metadata = {
  title: 'Dashboard: Return Decisions'
};

export default async function Page() {
  const session = await auth();
  const opts = {
    token: session?.user?.rustokToken ?? null,
    tenantSlug: session?.user?.tenantSlug ?? null,
    tenantId: session?.user?.tenantId ?? null
  };

  return <ReturnDecisionsTemplate opts={opts} />;
}
