import type { Pagination } from "@/types/pagination";
import {
  type CalculatedPaginationData,
  PaginationUtils,
} from "@/utils/pagination";
import { ArrowLeft } from "@phosphor-icons/react/dist/ssr/ArrowLeft";
import { useEffect, useMemo, useState } from "react";
import { PaginationButton, PaginationReticence } from "./pagination-button";
import { ArrowRight } from "@phosphor-icons/react/dist/ssr/ArrowRight";
import clsx from "clsx";

type TodoListingPaginationProps = {
  pagination: Pagination;
  uri: string;
  className?: string;
};

export function TodoListingPagination({
  pagination,
  uri,
  className,
}: TodoListingPaginationProps) {
  const [resolvedPagination, setResolvedPagination] =
    useState<CalculatedPaginationData>();

  const paginationData = useMemo(() => {
    return {
      currentPage: pagination.currentPage,
      totalOfPages: pagination.lastPage,
    };
  }, [pagination.currentPage, pagination.lastPage]);

  useEffect(() => {
    const resolvedPagination =
      PaginationUtils.getCalculatedPaginationData(paginationData);
    setResolvedPagination(resolvedPagination);
  }, [paginationData]);

  if (!resolvedPagination) return null;

  return (
    <div className={clsx("flex items-center gap-6", className && className)}>
      <PaginationButton
        page={1}
        uri={uri}
        disabled={pagination.currentPage - 1 <= 0}
      >
        <ArrowLeft size={24} weight="bold" />
      </PaginationButton>

      {!resolvedPagination.isLastPageVisible && (
        <>
          <PaginationButton page={1} uri={uri}>
            1
          </PaginationButton>
          <PaginationReticence />
        </>
      )}

      <div className="flex items-center gap-2">
        {resolvedPagination.calculatedPages.map((page) => (
          <PaginationButton
            key={`pagination-page-anchor-${page}`}
            page={page}
            uri={uri}
            disabled={page === pagination.currentPage}
          >
            {page}
          </PaginationButton>
        ))}
      </div>

      {!resolvedPagination.isLastPageVisible && (
        <>
          <PaginationReticence />
          <PaginationButton page={pagination.lastPage} uri={uri}>
            {pagination.lastPage}
          </PaginationButton>
        </>
      )}

      <PaginationButton
        page={1}
        uri={uri}
        disabled={pagination.currentPage + 1 > pagination.lastPage}
      >
        <ArrowRight size={24} weight="bold" />
      </PaginationButton>
    </div>
  );
}
