import { Alert } from "@/components/alert";
import { Button } from "@/components/button";
import { CheckboxTooltip } from "@/components/checkbox-tooltip";
import { H1 } from "@/components/h1";
import { Main } from "@/components/main";
import { useTypedPage } from "@/libs/hooks/use-typed-page";
import type { Todo } from "@/types/entities/todo";
import type { PaginatedEntitySet } from "@/types/paginated-entity-set";
import {
  type CompletedState,
  FilterByCompletedStatusCheckbox,
} from "@/ui/filter-by-completed-status-checkbox";
import { TodoListingCard } from "@/ui/todo-listing-card";
import { TodoListingPagination } from "@/ui/todo-listing-pagination";
import { Head, Link, router } from "@inertiajs/react";
import clsx from "clsx";
import { useCallback, useState } from "react";

type PageProps = {
  todos: PaginatedEntitySet<Todo>;
};

type PageFlash = {
  success?: string;
};

function getDefaultCompletedState(url: string, key: string): CompletedState {
  const urlCompletedState = new URL(url, window.location.href).searchParams.get(
    key,
  );

  switch (urlCompletedState) {
    case "true":
      return true;
    case "false":
      return false;
    default:
      return "indeterminate";
  }
}

export default function Index() {
  const page = useTypedPage<PageProps, PageFlash>();
  const todos = page.props.todos;

  const [query, setQuery] = useState<string | boolean | null>(null);
  const [completed, setCompleted] = useState(
    getDefaultCompletedState(page.url, "completed"),
  );

  const handleFilter = useCallback(() => {
    let queryString = {};

    if (query) queryString = { ...queryString, query };

    if (completed !== "indeterminate")
      queryString = { ...queryString, completed };

    let searchParams = new URLSearchParams(queryString).toString();
    searchParams = searchParams === "" ? "" : `?${searchParams}`;

    router.visit(`/${searchParams.toString()}`);
  }, [completed, query]);

  return (
    <>
      <Head>
        <title>Todo List</title>
        <meta
          name="description"
          content="Type whatever you think you gonna do later. We know you're not tho"
        />
      </Head>

      <Main>
        <header className="mb-6 flex items-center justify-between">
          <H1>Todo List</H1>

          <search className="flex items-center gap-3">
            <label className="bg-white border-2 p-4 transition-all ring-purple-500 ring-0 focus-within:ring-4">
              <input
                placeholder="Query something"
                className={clsx(
                  "placeholder:font-bold placeholder:text-gray-400 text-lg",
                  "border-none outline-none h-full w-full font-bold",
                )}
                onInput={(e) => setQuery(e.currentTarget.value)}
              />
            </label>

            <CheckboxTooltip
              message={
                <p>
                  Filter by to-do task completed status. Let it on
                  "indeterminate" mode for not applying the filter.
                </p>
              }
            >
              <FilterByCompletedStatusCheckbox
                setState={setCompleted}
                state={completed}
              />
            </CheckboxTooltip>

            <Button
              type="button"
              className="self-stretch "
              onClick={handleFilter}
            >
              Search
            </Button>
          </search>
        </header>

        {page.props.flash.success && (
          <Alert theme="success">{page.props.flash.success}</Alert>
        )}

        <section>
          {todos.pagination.totalItems > 0 ? (
            <div className="flex flex-col gap-3 mb-6">
              {todos.data.map(TodoListingCard)}
            </div>
          ) : (
            <Alert theme="warn">
              There are no to-do tasks! Try creating one 💅🏼✨
            </Alert>
          )}

          <div className="flex items-center justify-between">
            <Button
              asChild
              className="block w-fit"
              size="lg"
              theme="success"
              alwaysEnabled
            >
              <Link href="/new">Create a to-do task</Link>
            </Button>

            <TodoListingPagination
              pagination={todos.pagination}
              uri={page.url}
            />
          </div>
        </section>
      </Main>
    </>
  );
}
