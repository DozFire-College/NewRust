export interface Message{
    id: number;
    author: string;
    body: string;
    created_at: string;
    type: string;
    attachment: string | null;
}