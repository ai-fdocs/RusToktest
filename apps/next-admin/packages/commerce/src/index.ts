import type { NavItem } from '../../../src/shared/types';
import { graphqlRequest } from '../../../src/shared/api/graphql';
import { registerAdminModule } from '../../../src/modules/registry';

export const commerceNavItems: NavItem[] = [
  {
    title: 'Commerce',
    url: '#',
    i18nKey: 'commerce',
    group: 'modulePlugins',
    icon: 'modules',
    isActive: false,
    items: [
      {
        title: 'Shipping Profiles',
        url: '/dashboard/commerce/shipping-profiles',
        i18nKey: 'shippingProfiles'
      },
      {
        title: 'Cart Promotions',
        url: '/dashboard/commerce/promotions',
        i18nKey: 'cartPromotions'
      },
      {
        title: 'Order Changes',
        url: '/dashboard/commerce/order-changes',
        i18nKey: 'orderChanges'
      }
    ],
    access: { role: 'manager' }
  }
];

registerAdminModule({
  id: 'commerce',
  name: 'Commerce Management',
  navItems: commerceNavItems
});

// TYPES
export type ShippingProfileTranslation = {
  locale: string;
  name: string;
  description: string | null;
};

export type ShippingProfile = {
  id: string;
  slug: string;
  active: boolean;
  metadata: string;
  createdAt: string | null;
  updatedAt: string | null;
  translations: ShippingProfileTranslation[];
};

export type ShippingProfileList = {
  items: ShippingProfile[];
  total: number;
  page: number;
  perPage: number;
  hasNext: boolean;
};

export type CreateShippingProfileInput = {
  slug: string;
  translations: ShippingProfileTranslation[];
  metadata?: string | null;
};

export type UpdateShippingProfileInput = {
  slug?: string | null;
  translations?: ShippingProfileTranslation[] | null;
  metadata?: string | null;
};

export type CartPromotionPreview = {
  kind: string;
  scope: string;
  lineItemId: string | null;
  currencyCode: string;
  baseAmount: string;
  adjustmentAmount: string;
  adjustedAmount: string;
};

export type CartAdjustment = {
  id: string;
  source: string | null;
  scope: string;
  lineItemId: string | null;
  amount: string;
  metadata: string;
};

export type CartSnapshot = {
  id: string;
  currencyCode: string;
  shippingTotal: string;
  adjustmentTotal: string;
  totalAmount: string;
  adjustments: CartAdjustment[];
};

export type AdminCartPromotionInput = {
  sourceId: string;
  kind: string;
  scope: string;
  lineItemId?: string | null;
  discountPercent?: string | null;
  amount?: string | null;
  metadata?: string | null;
};

export type OrderChange = {
  id: string;
  orderId: string;
  changeType: string;
  status: string;
  description: string | null;
  preview: string;
  metadata: string;
  createdAt: string | null;
  updatedAt: string | null;
};

export type OrderChangeList = {
  items: OrderChange[];
  total: number;
  page: number;
  perPage: number;
  hasNext: boolean;
};

export type ExchangeDifferenceRefundInput = {
  amount: string;
  reason?: string | null;
  metadata?: string | null;
};

type GqlOpts = {
  token?: string | null;
  tenantSlug?: string | null;
  tenantId?: string | null;
};

// GRAPHQL QUERIES & MUTATIONS

const SHIPPING_PROFILES_QUERY = `
query CommerceAdminShippingProfiles($tenantId: UUID!, $filter: ShippingProfilesFilter) {
  shippingProfiles(tenantId: $tenantId, filter: $filter) {
    total
    page
    perPage
    hasNext
    items {
      id
      slug
      active
      metadata
      createdAt
      updatedAt
      translations {
        locale
        name
        description
      }
    }
  }
}`;

const SHIPPING_PROFILE_QUERY = `
query CommerceAdminShippingProfile($tenantId: UUID!, $id: UUID!) {
  shippingProfile(tenantId: $tenantId, id: $id) {
    id
    slug
    active
    metadata
    createdAt
    updatedAt
    translations {
      locale
      name
      description
    }
  }
}`;

const CREATE_SHIPPING_PROFILE_MUTATION = `
mutation CommerceAdminCreateShippingProfile($tenantId: UUID!, $input: CreateShippingProfileInputObject!) {
  createShippingProfile(tenantId: $tenantId, input: $input) {
    id
    slug
    active
    metadata
    createdAt
    updatedAt
    translations {
      locale
      name
      description
    }
  }
}`;

const UPDATE_SHIPPING_PROFILE_MUTATION = `
mutation CommerceAdminUpdateShippingProfile($tenantId: UUID!, $id: UUID!, $input: UpdateShippingProfileInputObject!) {
  updateShippingProfile(tenantId: $tenantId, id: $id, input: $input) {
    id
    slug
    active
    metadata
    createdAt
    updatedAt
    translations {
      locale
      name
      description
    }
  }
}`;

const DEACTIVATE_SHIPPING_PROFILE_MUTATION = `
mutation CommerceAdminDeactivateShippingProfile($tenantId: UUID!, $id: UUID!) {
  deactivateShippingProfile(tenantId: $tenantId, id: $id) {
    id
    slug
    active
  }
}`;

const REACTIVATE_SHIPPING_PROFILE_MUTATION = `
mutation CommerceAdminReactivateShippingProfile($tenantId: UUID!, $id: UUID!) {
  reactivateShippingProfile(tenantId: $tenantId, id: $id) {
    id
    slug
    active
  }
}`;

const PREVIEW_CART_PROMOTION_MUTATION = `
mutation CommerceAdminPreviewCartPromotion($tenantId: UUID!, $cartId: UUID!, $input: AdminCartPromotionInput!) {
  previewAdminCartPromotion(tenantId: $tenantId, cartId: $cartId, input: $input) {
    kind
    scope
    lineItemId
    currencyCode
    baseAmount
    adjustmentAmount
    adjustedAmount
  }
}`;

const APPLY_CART_PROMOTION_MUTATION = `
mutation CommerceAdminApplyCartPromotion($tenantId: UUID!, $cartId: UUID!, $input: AdminCartPromotionInput!) {
  applyAdminCartPromotion(tenantId: $tenantId, cartId: $cartId, input: $input) {
    id
    currencyCode
    shippingTotal
    adjustmentTotal
    totalAmount
    adjustments {
      id
      source
      scope
      lineItemId
      amount
      metadata
    }
  }
}`;

const ORDER_CHANGES_QUERY = `
query CommerceAdminOrderChanges($tenantId: UUID!, $filter: OrderChangesFilter) {
  orderChanges(tenantId: $tenantId, filter: $filter) {
    total
    page
    perPage
    hasNext
    items {
      id
      orderId
      changeType
      status
      description
      preview
      metadata
      createdAt
      updatedAt
    }
  }
}`;

const ORDER_CHANGE_QUERY = `
query CommerceAdminOrderChange($tenantId: UUID!, $id: UUID!) {
  orderChange(tenantId: $tenantId, id: $id) {
    id
    orderId
    changeType
    status
    description
    preview
    metadata
    createdAt
    updatedAt
  }
}`;

const APPLY_ORDER_CHANGE_MUTATION = `
mutation CommerceAdminApplyOrderChange($tenantId: UUID!, $id: UUID!, $input: ApplyOrderChangeInputObject!) {
  applyOrderChange(tenantId: $tenantId, id: $id, input: $input) {
    id
    status
    updatedAt
  }
}`;

const CANCEL_ORDER_CHANGE_MUTATION = `
mutation CommerceAdminCancelOrderChange($tenantId: UUID!, $id: UUID!, $input: CancelOrderChangeInputObject!) {
  cancelOrderChange(tenantId: $tenantId, id: $id, input: $input) {
    id
    status
    updatedAt
  }
}`;

// CLIENT METHODS

export async function listShippingProfiles(
  opts: GqlOpts,
  filter: { page?: number; perPage?: number; active?: boolean; search?: string } = {}
): Promise<ShippingProfileList> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage shipping profiles.');
  }

  const response = await graphqlRequest<
    { tenantId: string; filter: typeof filter },
    { shippingProfiles: ShippingProfileList }
  >(
    SHIPPING_PROFILES_QUERY,
    { tenantId: opts.tenantId, filter },
    opts.token,
    opts.tenantSlug
  );

  return response.shippingProfiles;
}

export async function getShippingProfile(
  opts: GqlOpts,
  id: string
): Promise<ShippingProfile | null> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage shipping profiles.');
  }

  const response = await graphqlRequest<
    { tenantId: string; id: string },
    { shippingProfile: ShippingProfile | null }
  >(
    SHIPPING_PROFILE_QUERY,
    { tenantId: opts.tenantId, id },
    opts.token,
    opts.tenantSlug
  );

  return response.shippingProfile;
}

export async function createShippingProfile(
  opts: GqlOpts,
  input: CreateShippingProfileInput
): Promise<ShippingProfile> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage shipping profiles.');
  }

  const response = await graphqlRequest<
    { tenantId: string; input: CreateShippingProfileInput },
    { createShippingProfile: ShippingProfile }
  >(
    CREATE_SHIPPING_PROFILE_MUTATION,
    { tenantId: opts.tenantId, input },
    opts.token,
    opts.tenantSlug
  );

  return response.createShippingProfile;
}

export async function updateShippingProfile(
  opts: GqlOpts,
  id: string,
  input: UpdateShippingProfileInput
): Promise<ShippingProfile> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage shipping profiles.');
  }

  const response = await graphqlRequest<
    { tenantId: string; id: string; input: UpdateShippingProfileInput },
    { updateShippingProfile: ShippingProfile }
  >(
    UPDATE_SHIPPING_PROFILE_MUTATION,
    { tenantId: opts.tenantId, id, input },
    opts.token,
    opts.tenantSlug
  );

  return response.updateShippingProfile;
}

export async function deactivateShippingProfile(
  opts: GqlOpts,
  id: string
): Promise<ShippingProfile> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage shipping profiles.');
  }

  const response = await graphqlRequest<
    { tenantId: string; id: string },
    { deactivateShippingProfile: ShippingProfile }
  >(
    DEACTIVATE_SHIPPING_PROFILE_MUTATION,
    { tenantId: opts.tenantId, id },
    opts.token,
    opts.tenantSlug
  );

  return response.deactivateShippingProfile;
}

export async function reactivateShippingProfile(
  opts: GqlOpts,
  id: string
): Promise<ShippingProfile> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage shipping profiles.');
  }

  const response = await graphqlRequest<
    { tenantId: string; id: string },
    { reactivateShippingProfile: ShippingProfile }
  >(
    REACTIVATE_SHIPPING_PROFILE_MUTATION,
    { tenantId: opts.tenantId, id },
    opts.token,
    opts.tenantSlug
  );

  return response.reactivateShippingProfile;
}

export async function previewCartPromotion(
  opts: GqlOpts,
  cartId: string,
  input: AdminCartPromotionInput
): Promise<CartPromotionPreview> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage promotions.');
  }

  const response = await graphqlRequest<
    { tenantId: string; cartId: string; input: AdminCartPromotionInput },
    { previewAdminCartPromotion: CartPromotionPreview }
  >(
    PREVIEW_CART_PROMOTION_MUTATION,
    { tenantId: opts.tenantId, cartId, input },
    opts.token,
    opts.tenantSlug
  );

  return response.previewAdminCartPromotion;
}

export async function applyCartPromotion(
  opts: GqlOpts,
  cartId: string,
  input: AdminCartPromotionInput
): Promise<CartSnapshot> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage promotions.');
  }

  const response = await graphqlRequest<
    { tenantId: string; cartId: string; input: AdminCartPromotionInput },
    { applyAdminCartPromotion: CartSnapshot }
  >(
    APPLY_CART_PROMOTION_MUTATION,
    { tenantId: opts.tenantId, cartId, input },
    opts.token,
    opts.tenantSlug
  );

  return response.applyAdminCartPromotion;
}

export async function listOrderChanges(
  opts: GqlOpts,
  filter: { page?: number; perPage?: number; orderId?: string; status?: string; changeType?: string } = {}
): Promise<OrderChangeList> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage order changes.');
  }

  const response = await graphqlRequest<
    { tenantId: string; filter: typeof filter },
    { orderChanges: OrderChangeList }
  >(
    ORDER_CHANGES_QUERY,
    { tenantId: opts.tenantId, filter },
    opts.token,
    opts.tenantSlug
  );

  return response.orderChanges;
}

export async function getOrderChange(
  opts: GqlOpts,
  id: string
): Promise<OrderChange | null> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage order changes.');
  }

  const response = await graphqlRequest<
    { tenantId: string; id: string },
    { orderChange: OrderChange | null }
  >(
    ORDER_CHANGE_QUERY,
    { tenantId: opts.tenantId, id },
    opts.token,
    opts.tenantSlug
  );

  return response.orderChange;
}

export async function applyOrderChange(
  opts: GqlOpts,
  id: string,
  metadata?: string | null,
  differenceRefund?: ExchangeDifferenceRefundInput | null
): Promise<OrderChange> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage order changes.');
  }

  const response = await graphqlRequest<
    { tenantId: string; id: string; input: { metadata?: string | null; differenceRefund?: ExchangeDifferenceRefundInput | null } },
    { applyOrderChange: OrderChange }
  >(
    APPLY_ORDER_CHANGE_MUTATION,
    { tenantId: opts.tenantId, id, input: { metadata, differenceRefund } },
    opts.token,
    opts.tenantSlug
  );

  return response.applyOrderChange;
}

export async function cancelOrderChange(
  opts: GqlOpts,
  id: string,
  reason?: string | null,
  metadata?: string | null
): Promise<OrderChange> {
  if (!opts.token || !opts.tenantSlug || !opts.tenantId) {
    throw new Error('Sign in again to manage order changes.');
  }

  const response = await graphqlRequest<
    { tenantId: string; id: string; input: { reason?: string | null; metadata?: string | null } },
    { cancelOrderChange: OrderChange }
  >(
    CANCEL_ORDER_CHANGE_MUTATION,
    { tenantId: opts.tenantId, id, input: { reason, metadata } },
    opts.token,
    opts.tenantSlug
  );

  return response.cancelOrderChange;
}
