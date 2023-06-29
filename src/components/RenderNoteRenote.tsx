import { Renote } from "../interfaces/note";
import RenderReactions from "./RenderReactions";

interface Props {
  note: Renote;
  full?: boolean;
}

function RenderNoteRenote({ note, full }: Props): JSX.Element {
  return (
    <div className="mt-1 min-h-[5em] relative">
      <div className="w-20 float-left">
        <img
          src={note.user.avatarUrl}
          className="rounded-full aspect-square object-cover"
        />
      </div>
      <div
        className={
          full ? "absolute top-0 right-0" : "absolute right-3 top-3"
        }
      >
        {note.modifiedCreatedAt}
      </div>
      <div className="ml-24">
        <div className="text-left">
          <span
            dangerouslySetInnerHTML={{
              __html: note.user.name,
            }}
          />
          <span className="text-gray-400 ml-2">
            {note.user.username}
            {note.user.host && `@${note.user.host}`}
          </span>
          {note.text && (
            <div
              className="mt-1"
              dangerouslySetInnerHTML={{
                __html: note.text,
              }}
            ></div>
          )}
        </div>
        {note.files && (
          <div className="flex flex-wrap">
            {note.files.map((file) => (
              <div
                key={file.id}
                className="m-1 relative w-64 h-36 bg-gray-500"
              >
                <img
                  key={file.id}
                  src={file.thumbnailUrl}
                  alt={file.name}
                  className="w-full h-full object-contain absolute"
                />
              </div>
            ))}
          </div>
        )}
        {note.modifiedEmojis && full && (
          <RenderReactions
            reactions={note.modifiedEmojis.reactions}
          />
        )}
      </div>
    </div>
  );
}

export default RenderNoteRenote;
