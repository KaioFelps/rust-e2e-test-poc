import { CheckboxTooltip } from "@/components/checkbox-tooltip";
import { Main } from "@/components/main";
import type { Todo } from "@/types/entities/todo";
import type { PaginatedEntitySet } from "@/types/paginated-entity-set";
import {
  type CompletedState,
  FilterByCompletedStatusCheckbox,
} from "@/ui/filter-by-completed-status-checkbox";
import { TodoListingCard } from "@/ui/todo-listing-card";
import { TodoListingPagination } from "@/ui/todo-listing-pagination";
import { Head, Link, router, usePage } from "@inertiajs/react";
import clsx from "clsx";
import { useCallback, useState } from "react";

type PageProps = {
  todos: PaginatedEntitySet<Todo>;
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
  const page = usePage<PageProps>();
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
          <h1 className="font-black text-8xl text-purple-500">Todo List</h1>

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

            <button
              type="button"
              className={clsx(
                "text-lg font-bold text-white bg-sky-500 px-6 self-stretch transition-all",
                "text-shadow-[3px_3px_0] text-shadow-blue-700 shadow-[4px_4px_0] shadow-black relative enabled:-top-1 enabled:-left-1",
                "enabled:hover:-top-2 enabled:hover:-left-2 enabled:hover:shadow-[6px_6px_0] enabled:hover:shadow-blue-800",
                "enabled:active:top-0 enabled:active:left-0 enabled:active:bg-sky-600 enabled:active:shadow-[2px_2px_0]",
                "disabled:bg-gray-400 disabled:shadow-[2px_2px_0]",
                "ring-inset ring-0 outline-none ring-blue-300 focus:ring-4",
              )}
              onClick={handleFilter}
            >
              Search
            </button>
          </search>
        </header>

        <section>
          {todos.pagination.totalItems > 0 ? (
            <div className="flex flex-col gap-3 mb-6">
              {todos.data.map(TodoListingCard)}
            </div>
          ) : (
            <span className="block mb-6 px-12 py-6 text-lg font-medium bg-yellow-300">
              There are no to-do tasks! Try creating one 💅🏼✨
            </span>
          )}

          <div className="flex items-center justify-between">
            <Link
              href="/new"
              className={clsx(
                "block w-fit font-black text-3xl px-8 py-4 bg-green-600 text-white shadow-[6px_6px_0] shadow-black",
                "text-shadow-[4px_4px_0] text-shadow-green-800 transition-all relative left-0 top-0",
                "hover:-left-1 hover:-top-1 hover:shadow-[10px_10px_0] hover:shadow-green-900",
                "active:top-1 active:left-1 active:shadow-[2px_2px_0] active:shadow-black active:bg-green-700",
                "outline-none ring-inset ring-0 ring-green-400 focus:ring-4",
              )}
            >
              Create a to-do task
            </Link>

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
