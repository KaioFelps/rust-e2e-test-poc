import { Button } from "@/components/button";
import Form from "@/components/form";
import { H1 } from "@/components/h1";
import { Main } from "@/components/main";
import { useForm } from "@inertiajs/react";
import type { FormEvent } from "react";

type NewTodoFormData = {
  title: string;
  content: string;
};

export default function New() {
  const { data, setData, errors, post } = useForm<NewTodoFormData>();

  const handleCreateNewTodo = (event: FormEvent) => {
    event.preventDefault();
    post("/create");
  };

  return (
    <Main>
      <H1 className="mb-6 capitalize">New to-do task</H1>

      <Form.Root onSubmit={handleCreateNewTodo}>
        <Form.TextInput
          label="Title"
          name="title"
          placeholder="Whatcha gotta do?"
          setValue={(title) => setData({ ...data, title })}
          error={errors.title}
        />

        <Form.Textarea
          label="Description"
          name="content"
          setValue={(content) => setData({ ...data, content })}
          placeholder="Tell urself a little more about this chore"
          error={errors.content}
        />

        <Button type="submit" theme="success" size="lg" className="self-start">
          Create
        </Button>
      </Form.Root>
    </Main>
  );
}
