<script lang="ts">
  import { Button } from "@/lib/components/ui/button";
  import * as Field from "@/lib/components/ui/field";
  import { Input } from "@/lib/components/ui/input";

  let { handleFinalSubmit, apiKeyValue, errors, isSaving, showPasswordFields } =
    $props();
</script>

<form class="w-full max-w-sm" onsubmit={handleFinalSubmit}>
  <Field.Group>
    <input type="hidden" name="apiKey" value={apiKeyValue} />

    <div
      class="mb-4 p-4 bg-yellow-50 dark:bg-yellow-950 border border-yellow-200 dark:border-yellow-800 rounded-lg"
    >
      <p class="text-sm text-yellow-800 dark:text-yellow-200">
        <strong>Important:</strong> This password encrypts your API key. There's
        no way to recover it if you forget it.
      </p>
    </div>

    <Field.Field>
      <Field.Label for="password">Create Master Password</Field.Label>
      <Input
        id="password"
        name="password"
        type="password"
        placeholder="Enter a strong password"
        required
        disabled={isSaving}
        aria-invalid={!!errors.password}
        autofocus
      />
      {#if errors.password}
        <Field.Error>{errors.password}</Field.Error>
      {/if}
      <Field.Description>
        At least 8 characters. Make it memorable!
      </Field.Description>
    </Field.Field>

    <Field.Field>
      <Field.Label for="confirmPassword">Confirm Password</Field.Label>
      <Input
        id="confirmPassword"
        name="confirmPassword"
        type="password"
        placeholder="Confirm your password"
        required
        disabled={isSaving}
        aria-invalid={!!errors.confirmPassword}
      />
      {#if errors.confirmPassword}
        <Field.Error>{errors.confirmPassword}</Field.Error>
      {/if}
    </Field.Field>

    <Field.Field>
      <div class="flex gap-2">
        <Button
          type="button"
          variant="outline"
          onclick={() => (showPasswordFields = false)}
          disabled={isSaving}
          class="flex-1"
        >
          Back
        </Button>
        <Button type="submit" disabled={isSaving} class="flex-1">
          {isSaving ? "Saving..." : "Complete Setup"}
        </Button>
      </div>
    </Field.Field>
  </Field.Group>
</form>
