import { Note } from "../interfaces/note";

interface Props {
  note: Note;
}

function RenderNote({ note }: Props): JSX.Element {
  return (
    <div className="min-h-[7em] border-2 p-3 mr-1 ml-1 border-pink-100 border-dashed relative">
      <div className="w-20 float-left">
        <img
          src={note.user.avatarUrl}
          className="rounded-full aspect-square object-cover"
        />
      </div>
      <div className="absolute right-3 top-3">
        {note.modifiedCreatedAt}
      </div>
      <div className="ml-24">
        <div className="text-left">
          {note.user.name ?? note.user.username}
          <span className="text-gray-400 ml-2">
            {note.user.username}
            {note.user.host && `@${note.user.host}`}
          </span>
          {note.text && <div className="mt-1">{note.text}</div>}
        </div>
        {note.files && (
          <div className="flex flex-wrap">
            {note.files.map((file, index) => (
              <div
                key={index}
                className="m-1 relative w-64 h-36 bg-gray-500"
              >
                <img
                  key={index}
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
            <div className="w-20 float-left">
              <img
                src={note.renote.user.avatarUrl}
                className="rounded-full aspect-square object-cover"
              />
            </div>
            <div className="absolute right-3 top-3">
              {note.renote.modifiedCreatedAt}
            </div>
            <div className="ml-24">
              <div className="text-left">
                {note.renote.user.name ?? note.renote.user.username}
                <span className="text-gray-400 ml-2">
                  {note.renote.user.username}
                  {note.renote.user.host &&
                    `@${note.renote.user.host}`}
                </span>
                {note.renote.text && (
                  <div className="mt-1">{note.renote.text}</div>
                )}
              </div>
              {note.renote.files && (
                <div className="flex flex-wrap">
                  {note.renote.files.map((file, index) => (
                    <div
                      key={index}
                      className="m-1 relative w-64 h-36 bg-gray-500"
                    >
                      <img
                        key={index}
                        src={file.thumbnailUrl}
                        alt={file.name}
                        className="w-full h-full object-contain absolute"
                      />
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}
        {note.modifiedEmojis && (
          <div className="flex gap-1">
            {note.modifiedEmojis.reactions.map((reaction) => (
              <div
                key={reaction.name}
                className="flex items-center mt-2"
              >
                {reaction.url ? (
                  <img
                    className="w-auto h-5 max-w-full mr-1"
                    src={reaction.url}
                    alt={reaction.name}
                  />
                ) : (
                  <span className="mr-1">{reaction.name}</span>
                )}
                {reaction.count}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default RenderNote;
