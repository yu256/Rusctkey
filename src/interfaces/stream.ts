import { Note } from "./note";

export interface streamingBody {
  type: string;
  body: body;
}

interface body {
  id: string;
  type: string;
  body: Note;
}
