import { Note } from "../interfaces/note";
import RenderFiles from "./RenderFiles";
import RenderNoteRenote from "./RenderNoteRenote";
import RenderReactions from "./RenderReactions";
import RenderTime from "./RenderTime";

interface Props {
  note: Note;
}

function RenderNote({ note }: Props): JSX.Element {
  const { createdAt, user, text, modifiedEmojis, files, renote } =
    note;
  return !text && !files.length && renote ? (
    <li className="min-h-[7em] border-2 p-3 mr-1 ml-1 border-pink-100 border-dashed relative">
      <div className="w-6 float-left">
        <img
          src={user.avatarUrl}
          className="rounded-full aspect-square object-cover"
        />
      </div>
      <div className="absolute right-3 top-3">
        <RenderTime createdAt={createdAt} />
      </div>
      <span dangerouslySetInnerHTML={{ __html: user.name }} />
      がRenote
      <RenderNoteRenote note={renote} full />
    </li>
  ) : (
    <li className="min-h-[7em] border-2 p-3 mr-1 ml-1 border-pink-100 border-dashed relative">
      <div className="w-20 float-left">
        <img
          src={user.avatarUrl}
          className="rounded-full aspect-square object-cover"
        />
      </div>
      <div className="absolute right-3 top-3">
        <RenderTime createdAt={createdAt} />
      </div>
      <div className="ml-24">
        <div className="text-left">
          <span dangerouslySetInnerHTML={{ __html: user.name }} />
          <span className="text-gray-400 ml-2">
            {user.username}
            {user.host && `@${user.host}`}
          </span>
          {text && (
            <div
              className="mt-1"
              dangerouslySetInnerHTML={{ __html: text }}
            />
          )}
        </div>
        {!!files.length && <RenderFiles files={files} />}
        {renote && (
          <div className="min-h-[7em] border-2 p-3 ml-1 mr-1 rounded-3xl border-black border-dashed relative">
            <RenderNoteRenote note={renote} />
          </div>
        )}
        {modifiedEmojis && (
          <RenderReactions reactions={modifiedEmojis.reactions} />
        )}
      </div>
    </li>
  );
}

export default RenderNote;
