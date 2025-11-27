<script lang="ts">
  import { Input } from "@/lib/components/ui/input";
  import * as Select from "$lib/components/ui/select";
  import * as Field from "@/lib/components/ui/field";
  import { InputTag } from "@/lib/components/ui/input-tag";
  import type { Tag } from "@/lib/components/ui/input-tag/input-tag.svelte";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";
  import { z } from "zod/v4";
  import { Editor } from "@/lib/components/editor";
  import { X } from "@lucide/svelte";

  const emailComposerSchema = z.object({
    from: z.string().nonempty("From is required"),
    to: z
      .array(
        z.email({
          message: "Invalid email address",
        }),
      )
      .nonempty("At least one recipient is required"),
    subject: z.string().nonempty("Subject is required"),
    cc: z.array(z.email()).optional(),
    bcc: z.array(z.email()).optional(),
    replyTo: z.email().optional().nullable(),
    content: z.any(),
    files: z.any().optional(),
  });

  const fromEmailOptions = [
    "Cristhian <c@rxtsel.dev>",
    "Contact <contact@rxtsel.dev>",
    "Hello <hello@rxtsel.dev>",
  ];

  let toEmails = $state<Tag[]>([]);
  let from = $state<string>(fromEmailOptions[0]);
  let content = $state<string>("");
  let files = $state<File[]>([]);

  let isCcEnabled = $state<boolean>(false);
  let isBccEnabled = $state<boolean>(false);
  let isReplyToEnabled = $state<boolean>(false);
  let ccEmails = $state<Tag[]>([]);
  let bccEmails = $state<Tag[]>([]);

  function handleSubmit(event: Event) {
    event.preventDefault();
    const form = event.currentTarget as HTMLFormElement;
    const formData = new FormData(form);

    // Get all to emails
    const toEmailsList = formData.getAll("to");

    const data = {
      from: formData.get("from"),
      to: toEmailsList,
      subject: formData.get("subject"),
      bcc: isBccEnabled ? formData.getAll("bcc") : [],
      cc: isCcEnabled ? formData.getAll("cc") : [],
      replyTo: isReplyToEnabled ? formData.get("replyTo") : null,
      content,
      files,
    };

    const rawData = emailComposerSchema.safeParse(data);

    console.log({ rawData });
  }
</script>

<form
  class="h-full w-full overflow-hidden"
  onsubmit={handleSubmit}
  id="compose-email-form"
>
  <Field.Group class="h-full">
    <!-- From and Group buttons -->
    <div class="flex gap-1 items-end">
      <Field.Field class="flex-1">
        <Field.Label for="from">From</Field.Label>
        <Select.Root type="single" name="from" bind:value={from}>
          <Select.Trigger class="w-[180px]" id="from">
            {#if from}
              {from}
            {:else}
              Select an email
            {/if}
          </Select.Trigger>
          <Select.Content>
            {#each fromEmailOptions as option}
              <Select.Item value={option}>{option}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </Field.Field>
      <Field.Field class="shrink-0 w-fit">
        <ToggleGroup.Root type="multiple" variant="outline">
          <ToggleGroup.Item
            value="cc"
            onclick={() => (isCcEnabled = !isCcEnabled)}>Cc</ToggleGroup.Item
          >
          <ToggleGroup.Item
            value="bcc"
            onclick={() => (isBccEnabled = !isBccEnabled)}>Bcc</ToggleGroup.Item
          >
          <ToggleGroup.Item
            value="replyTo"
            class="px-5"
            onclick={() => (isReplyToEnabled = !isReplyToEnabled)}
          >
            Reply To
          </ToggleGroup.Item>
        </ToggleGroup.Root>
      </Field.Field>
    </div>

    <!-- To and cc -->
    <div class="flex gap-1">
      <Field.Field class="flex-1">
        <Field.Label for="to">To</Field.Label>
        <InputTag
          id="to"
          type="email"
          bind:value={toEmails}
          name="to"
          placeholder="Add recipient email"
        />
      </Field.Field>
      {#if isCcEnabled}
        <Field.Field class="flex-1">
          <Field.Label for="cc">Cc</Field.Label>
          <InputTag
            id="cc"
            type="email"
            bind:value={ccEmails}
            name="cc"
            placeholder="Add Cc email"
          />
        </Field.Field>
      {/if}
    </div>

    <!-- Bcc and Reply-To -->
    {#if isBccEnabled || isReplyToEnabled}
      <div class="flex gap-1">
        {#if isBccEnabled}
          <Field.Field class="flex-1">
            <Field.Label for="bcc">Bcc</Field.Label>
            <InputTag
              id="bcc"
              type="email"
              bind:value={bccEmails}
              name="bcc"
              placeholder="Add Bcc email"
            />
          </Field.Field>
        {/if}
        {#if isReplyToEnabled}
          <Field.Field class="flex-1">
            <Field.Label for="replyTo">Reply To</Field.Label>
            <Input
              id="replyTo"
              type="email"
              name="replyTo"
              placeholder="Reply-To email"
            />
          </Field.Field>
        {/if}
      </div>
    {/if}

    <Field.Field>
      <Field.Label for="subject">Subject</Field.Label>
      <Input id="subject" type="text" name="subject" placeholder="Subject" />
    </Field.Field>
    <Field.Field class="h-full">
      <Field.Label for="content">Content</Field.Label>
      <div class="h-full flex flex-col">
        <Editor bind:content bind:files />
        {#if files.length}
          <div class="border border-t-0 min-h-20 max-h-30 rounded-b-md z-10">
            <ul
              class="py-2 pr-2 pl-7 flex list-disc flex-wrap flex-col gap-1 overflow-y-auto h-full"
            >
              {#each files as file, index}
                <li class="flex items-center gap-1">
                  <span class="truncate text-sm text-muted-foreground"
                    >{file.name}</span
                  >
                  <button
                    class="size-fit group"
                    type="button"
                    onclick={() => {
                      files = files.filter((_, i) => i !== index);
                    }}
                  >
                    <X
                      class="size-3 text-muted-foreground group-hover:text-black"
                    />
                  </button>
                </li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    </Field.Field>
  </Field.Group>
</form>
