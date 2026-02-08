export type StoreSnapshot<T> = {
  state: T;
};

export type StoreUpdate<T> = {
  previous: T;
  next: T;
};
