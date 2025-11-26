<script lang="ts">
  import { Button } from "@/lib/components/ui/button";
  import * as Field from "@/lib/components/ui/field";
  import { Input } from "@/lib/components/ui/input";

  let { handleApiKeyNext, apiKeyValue = $bindable(), errors } = $props();
</script>

<form class="w-full max-w-sm" onsubmit={handleApiKeyNext}>
  <Field.Group>
    <Field.Field>
      <Field.Label for="apiKey">Resend API Key</Field.Label>
      <Input
        id="apiKey"
        name="apiKey"
        bind:value={apiKeyValue}
        placeholder="re_123abc456def789gho0tevecovc94htoi"
        required
        aria-invalid={!!errors.apiKey}
        autofocus
      />
      {#if errors.apiKey}
        <Field.Error>{errors.apiKey}</Field.Error>
      {/if}
      <Field.Description>
        Enter your Resend API key to get started. You can find it in your
        <a
          href="https://resend.com/api-keys"
          target="_blank"
          rel="noopener
                noreferrer"
          class="underline hover:text-foreground"
        >
          Resend dashboard
        </a>.
      </Field.Description>
    </Field.Field>
    <Field.Field>
      <Button type="submit" class="w-full">Continue</Button>
    </Field.Field>
  </Field.Group>
</form>
