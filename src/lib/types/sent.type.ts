export interface SentEmail {
  id: string;
  from: string;
  to: string[];
  subject: string;
  html: string | null;
  created_at: string;
  cc: string[];
  bcc: string[];
  reply_to: string | null;
}
