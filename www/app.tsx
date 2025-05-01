import "./index.css";

import React from "react";
import { createInertiaApp } from "@inertiajs/react";
import { createRoot } from "react-dom/client";
import { DefaultLayout } from "./layouts/default";
import { type JSX } from "react";

createInertiaApp({
    progress: { includeCSS: true, color: "var(--color-purple-500)" },
    
    title: (title: string) => title ?? "Inertia Rust",
    
    resolve: (pageName: string) => {
        const pages = import.meta.glob("./pages/**/*.tsx", { eager: true });
        const page = pages[`./pages/${ pageName }.tsx`] as ComponentPage;

        if (!page) throw new Error(`Could not find page ${pageName}.`);

        page!.default.layout ??= (page: JSX.Element) => <DefaultLayout>{page}</DefaultLayout>;

        return page;
    },
    
    setup: ({ App, el,props }) => {
        createRoot(el).render(<App {...props} />)
    },
})

type ComponentPage = {
    default: {
        layout?: (page: JSX.Element) => JSX.Element
    }
}
