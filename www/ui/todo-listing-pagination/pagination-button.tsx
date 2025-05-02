import { PaginationUtils } from "@/utils/pagination";
import { router } from "@inertiajs/react";
import clsx from "clsx";
import { PropsWithChildren, useCallback, useMemo } from "react"

type PaginationButtonProps = {
    page: number;
    uri: string;
    disabled?: boolean;
}

export function PaginationButton({
    children,
    disabled = false,
    page,
    uri
}: PropsWithChildren<PaginationButtonProps>) {
    const url = useMemo(() => PaginationUtils.resolvePageUri({uri, page}), [page, uri])
    const handleGoToPage = useCallback(() => {
        router.visit(url);
    }, [url]);

    return (
        <button
            className={clsx(
                "size-16 grid place-items-center text-black text-xl transition-all",
                "border-2 border-black bg-purple-300",
                "enabled:hover:bg-purple-400 enabled:active:bg-purple-600",
                "outline-none ring-purple-200 ring-0 focus:ring-4",
                "disabled:saturate-0 opacity-70"
            )}
            onClick={handleGoToPage}
            disabled={disabled}
        >
            {children}
        </button>
    )
}

export function PaginationReticence() {
    return (
        <span
            className={clsx(
                "size-16 grid place-items-center text-black text-xl transition-all",
                "border-2 border-black bg-purple-300",
                "enabled:hover:bg-purple-400 enabled:active:bg-purple-600",
                "outline-none ring-purple-200 ring-0 focus:ring-4",
                "disabled:saturate-0 opacity-70"
            )}
        >
            ...
        </span>
    )
}