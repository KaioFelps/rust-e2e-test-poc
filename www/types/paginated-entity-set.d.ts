import type { Pagination } from "./pagination";

export type PaginatedEntitySet<T> = {
  data: T[];
  pagination: Pagination;
};
