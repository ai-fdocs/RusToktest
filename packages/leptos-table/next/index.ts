export type SortDirection = "asc" | "desc";

export type SortRule = {
  field: string;
  direction: SortDirection;
};

export type FilterRule = {
  field: string;
  value: string;
};

export type TableState = {
  page: number;
  pageSize: number;
  sort: SortRule[];
  filters: FilterRule[];
};

export const TableState = {
  create: (page: number, pageSize: number): TableState => ({
    page,
    pageSize,
    sort: [],
    filters: [],
  }),
  withSort: (state: TableState, sort: SortRule[]): TableState => ({
    ...state,
    sort,
  }),
  withFilters: (state: TableState, filters: FilterRule[]): TableState => ({
    ...state,
    filters,
  }),
};
