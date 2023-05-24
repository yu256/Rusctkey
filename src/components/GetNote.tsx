import { invoke } from "@tauri-apps/api";
import { useState } from "react";

function GetNote() {
  const [noteId, setNoteId] = useState("");
  const [note, getNote] = useState<Note>();

  async function get() {
    getNote(await invoke<Note>("get_note", { noteId }));
  }

  function isCustomEmoji(reaction: string): boolean {
    return reaction.startsWith(":");
  }

  function getCustomEmojiURL(reaction: string): {
    url: string;
    alt: string;
  } {
    const emoji = note!.emojis.find(
      (emoji) => emoji.name === reaction.slice(1, -1)
    );
    return { url: emoji!.url, alt: reaction };
  }

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          get();
        }}
      >
        <input
          onChange={(e) => setNoteId(e.currentTarget.value)}
          placeholder="ノートのURL"
        />
        <button type="submit">Get</button>
      </form>
      {note && (
        <div className="note">
          <div className="icon">
            <img src={note.user.avatarUrl} />
          </div>
          <div className="notetext">
            {note.user.name} {note.user.username}@{note.user.host}
            <br />
            {note.text}
            <br />
            {note.createdAt}
            <br />
            {note.reactions && (
              <div className="emojis">
                {Object.entries(note.reactions).map(
                  ([reaction, count]) => (
                    <div key={reaction}>
                      {isCustomEmoji(reaction) ? (
                        <img
                          src={getCustomEmojiURL(reaction).url}
                          alt={getCustomEmojiURL(reaction).alt}
                        />
                      ) : (
                        <span>{reaction}</span>
                      )}
                      {count}
                    </div>
                  )
                )}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

interface Note {
  createdAt: string;
  user: User;
  text: string;
  reactions: Map<string, number>;
  emojis: Emoji[];
}

interface Emoji {
  name: string;
  url: string;
}

interface User {
  username: string;
  host: string | null;
  name: string;
  avatarUrl: string;
  instance: Instance;
  onlineStatus: string;
}

interface Instance {
  name: string;
  softwareName: string;
  softwareVersion: string;
  iconUrl: string;
  faviconUrl: string;
  themeColor: string;
}

export default GetNote;
