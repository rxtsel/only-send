import z from "zod/v4";

export const emailComposerSchema = z.object({
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
  content: z.string().nonempty("Content is required"),
  files: z.any().optional(),
  messageId: z.string().optional().nullable(),
});
