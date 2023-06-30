import { Note } from "../interfaces/note";
import RenderNoteRenote from "./RenderNoteRenote";
import RenderReactions from "./RenderReactions";
import RenderTime from "./RenderTime";

interface Props {
  note: Note;
}

function RenderNote({ note }: Props): JSX.Element {
  return !note.text && !note.files[0] && note.renote ? (
    <li className="min-h-[7em] border-2 p-3 mr-1 ml-1 border-pink-100 border-dashed relative">
      <div className="w-6 float-left">
        <img
          src={note.user.avatarUrl}
          className="rounded-full aspect-square object-cover"
        />
      </div>
      <div className="absolute right-3 top-3">
        <RenderTime createdAt={note.createdAt} />
      </div>
      <span dangerouslySetInnerHTML={{ __html: note.user.name }} />
      がRenote
      <RenderNoteRenote note={note.renote} full={true} />
    </li>
  ) : (
    <li className="min-h-[7em] border-2 p-3 mr-1 ml-1 border-pink-100 border-dashed relative">
      <div className="w-20 float-left">
        <img
          src={note.user.avatarUrl}
          className="rounded-full aspect-square object-cover"
        />
      </div>
      <div className="absolute right-3 top-3">
        <RenderTime createdAt={note.createdAt} />
      </div>
      <div className="ml-24">
        <div className="text-left">
          <span
            dangerouslySetInnerHTML={{ __html: note.user.name }}
          />
          <span className="text-gray-400 ml-2">
            {note.user.username}
            {note.user.host && `@${note.user.host}`}
          </span>
          {note.text && (
            <div
              className="mt-1"
              dangerouslySetInnerHTML={{ __html: note.text }}
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
        {note.renote && (
          <div className="min-h-[7em] border-2 p-3 ml-1 mr-1 rounded-3xl border-black border-dashed relative">
            <RenderNoteRenote note={note.renote} />
          </div>
        )}
        {note.modifiedEmojis && (
          <RenderReactions
            reactions={note.modifiedEmojis.reactions}
          />
        )}
      </div>
    </li>
  );
}

export default RenderNote;
