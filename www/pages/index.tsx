import { Todo } from "@/types/entities/todo";
import { PaginatedEntitySet } from "@/types/paginated-entity-set";
import { TodoListingCard } from "@/ui/todo-listing-card";
import { Head, Link } from "@inertiajs/react";
import clsx from "clsx";
import { useEffect, useState } from "react";

type PageProps = {
  todos: PaginatedEntitySet<Todo>
}
export default function Index({ todos }: PageProps) {
  const [mockedTodos, setMockedTodos] = useState<Todo[]>([]);

  useEffect(() => {
    setMockedTodos([
      ...todos.data,
      { id: 1, completed: false, content: "Foo bar foooooo", createdAt: new Date(), title: "Foo!!!!" } satisfies Todo
    ]);
  }, []);

  return (
    <>
      <Head>
        <title>Todo List</title>
        <meta name="description"  content="Type whatever you think you gonna do later. We know you're not tho" />
      </Head>

      <main>
        <header className="mb-6 flex items-center justify-between">
          <h1 className="font-black text-8xl text-purple-500">
            Todo List
          </h1>

          <filter className="flex items-center gap-3">
            <label className="bg-white border-2 p-4">
              <input
                placeholder="Query something"
                className="placeholder:font-bold placeholder:text-gray-400 text-lg"
              />
            </label>
            <button
              className={clsx(
                "text-lg font-bold text-white bg-sky-500 px-6 self-stretch transition-all",
                "shadow-[4px_4px_0] shadow-black relative enabled:-top-1 enabled:-left-1",
                "enabled:hover:-top-2 enabled:hover:-left-2 enabled:hover:shadow-[6px_6px_0] enabled:hover:shadow-blue-800",
                "enabled:active:top-0 enabled:active:left-0 enabled:active:bg-sky-600 enabled:active:shadow-[2px_2px_0]",
                "disabled:bg-gray-400 disabled:shadow-[2px_2px_0]"
              )}
            >
              Search
            </button>
          </filter>
        </header>

        <section>
          {mockedTodos.length > 0 ? (
            <div className="flex flex-col gap-3 mb-6">
              {mockedTodos.map(TodoListingCard)}
            </div>
          ) : (
            <span className="block mb-6 w-fit px-12 py-6 text-lg font-medium bg-yellow-300">
              There are no to-do tasks! Try creating one 💅🏼✨
            </span>
          )}
            <Link
              href="/new"
              className={clsx(
                "block w-fit font-black text-3xl px-8 py-4 bg-green-600 text-white shadow-[6px_6px_0] shadow-black",
                "text-shadow-[4px_4px_0] text-shadow-green-800 transition-all relative left-0 top-0",
                "hover:-left-1 hover:-top-1 hover:shadow-[10px_10px_0] hover:shadow-green-900",
                "active:top-1 active:left-1 active:shadow-[2px_2px_0] active:shadow-black active:bg-green-700"
              )}
            >
              Create a to-do task
            </Link>
        </section>
      </main>
    </>
  )
}
