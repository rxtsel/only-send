<script lang="ts">
  import { page } from "$app/state";
  import AppSidebar from "@/lib/components/app-sidebar.svelte";
  import * as Breadcrumb from "@/lib/components/ui/breadcrumb";
  import { Separator } from "@/lib/components/ui/separator";
  import * as Sidebar from "@/lib/components/ui/sidebar";
  import { Send } from "@lucide/svelte";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import { buttonVariants } from "$lib/components/ui/button";

  let { children } = $props();

  let isConfirmDialogOpen = $state(false);
</script>

<Sidebar.Provider style="--sidebar-width: 450px;">
  <AppSidebar />
  <Sidebar.Inset>
    <header
      class="bg-background sticky top-0 flex shrink-0 items-center gap-2 border-b p-4"
    >
      {#if page.url.pathname !== "/mail/composer"}
        <Sidebar.Trigger class="-ms-1" />

        <Separator
          orientation="vertical"
          class="me-2 data-[orientation=vertical]:h-4"
        />
        <Breadcrumb.Root>
          <Breadcrumb.List>
            <Breadcrumb.Item class="hidden md:block">
              <Breadcrumb.Link href="##">All Inboxes</Breadcrumb.Link>
            </Breadcrumb.Item>
            <Breadcrumb.Separator class="hidden md:block" />
            <Breadcrumb.Item>
              <Breadcrumb.Page>Inbox</Breadcrumb.Page>
            </Breadcrumb.Item>
          </Breadcrumb.List>
        </Breadcrumb.Root>
      {:else}
        <div class="flex w-full justify-between items-center gap-2">
          <h2 class="text-lg font-semibold">New Email</h2>

          {@render ConfirmDialog()}
        </div>
      {/if}
    </header>
    <div class="flex flex-1 flex-col gap-4 p-4">
      {@render children()}
    </div>
  </Sidebar.Inset>
</Sidebar.Provider>

{#snippet ConfirmDialog()}
  <AlertDialog.Root bind:open={isConfirmDialogOpen}>
    <AlertDialog.Trigger class={buttonVariants({ variant: "default" })}>
      <Send />
      Send
    </AlertDialog.Trigger>
    <AlertDialog.Content>
      <AlertDialog.Header>
        <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
        <AlertDialog.Description>
          This action cannot be undone. This will permanently send your email
        </AlertDialog.Description>
      </AlertDialog.Header>
      <AlertDialog.Footer>
        <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
        <AlertDialog.Action
          form="compose-email-form"
          type="submit"
          onclick={() => (isConfirmDialogOpen = false)}
        >
          Continue
        </AlertDialog.Action>
      </AlertDialog.Footer>
    </AlertDialog.Content>
  </AlertDialog.Root>
{/snippet}
