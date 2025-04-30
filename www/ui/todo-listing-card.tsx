import { Todo } from "@/types/entities/todo";
import clsx from "clsx";

type TodoListingCardProps = Todo;

export function TodoListingCard({completed, content, createdAt, id, title}: TodoListingCardProps) {
    return (
        <article
            key={"todo-listing-card-for-" + id}
            className="bg-white shadow-[4px_4px_0] shadow-yellow-700/40 border-2"
        >
            <h2 className="font-bold text-3xl text-amber-500 px-12 py-4 border-b-2 border-black">
                {title}
            </h2>
            <p
                className={clsx(
                    completed && "decoration-2 line-through",
                    "px-12 py-8 text-xl font-medium"
                )}
            >
                {content}
            </p>
            <footer className="px-12 py-4 border-t-2 border-black">
                <span className="font-bold text-gray-400">Criado em {new Date(createdAt).toLocaleDateString("pt-BR")}</span>
            </footer>
        </article>
    );
}