import { Note } from "../interfaces/note";

interface Props {
  note: Note;
}

function RenderNote({ note }: Props): JSX.Element {
  function isCustomEmoji(reaction: string): boolean {
    return reaction.startsWith(":");
  }

  function getCustomEmojiURL(reaction: string): {
    url: string;
    alt: string;
  } {
    const emoji = note.emojis.find(
      (emoji) => emoji.name === reaction.slice(1, -1)
    );
    return { url: emoji!.url, alt: reaction };
  }

  function getCustomEmojiURLinText(
    name: string,
    isUsername: boolean
  ): string | undefined {
    const matchedEmoji = isUsername
      ? note.user.emojis.find((emoji) => emoji.name === name)
      : note.emojis.find((emoji) => emoji.name === name);
    return matchedEmoji ? matchedEmoji.url : undefined;
  }

  const TextWithImages = (text: string, isUsername: boolean) => {
    const parseText = (text: string) => {
      const regex = /:(.*?):/g;
      const matches = text.match(regex);

      if (matches) {
        const parts = text.split(regex);
        return parts.map((part, index) => {
          if (matches.includes(`:${part}:`)) {
            return (
              <img
                key={index}
                src={getCustomEmojiURLinText(part, isUsername)}
                alt=":${part}:"
                className="h-4 inline"
              />
            );
          } else {
            return part;
          }
        });
      } else {
        return text;
      }
    };

    return <div>{parseText(text)}</div>;
  };

  return (
    <div className="border-2 p-5 ml-1 mr-1 rounded-3xl border-black border-dashed">
      <div className="w-20 float-left">
        <img src={note.user.avatarUrl} className="rounded-full" />
      </div>
      <div className="ml-10">
        {TextWithImages(note.user.name, true)} {note.user.username}
        {note.user.host && `@${note.user.host}`}
        <br />
        {TextWithImages(note.text, false)}
        <br />
        {note.files && (
          <div className="flex">
            {note.files.map((file, index) => (
              <div>
                <img
                  key={index}
                  src={file.url}
                  alt={file.name}
                  className="w-full h-full object-cover"
                />
              </div>
            ))}
          </div>
        )}
        <br />
        {note.createdAt}
        <br />
        {note.reactions && (
          <div className="flex gap-1">
            {Object.entries(note.reactions).map(
              ([reaction, count]) => (
                <div
                  key={reaction}
                  className="flex items-center mt-2"
                >
                  {isCustomEmoji(reaction) ? (
                    <img
                      className="w-auto h-5 max-w-full mr-1"
                      src={getCustomEmojiURL(reaction).url}
                      alt={getCustomEmojiURL(reaction).alt}
                    />
                  ) : (
                    <span className="mr-1">{reaction}</span>
                  )}
                  {count}
                </div>
              )
            )}
          </div>
        )}
      </div>
    </div>
  );
}

export default RenderNote;
