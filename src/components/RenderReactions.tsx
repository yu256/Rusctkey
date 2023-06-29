import { Reaction } from "../interfaces/note";

interface Props {
  reactions: Reaction[];
}

function RenderReactions({ reactions }: Props) {
  return (
    <div className="flex gap-1">
      {reactions.map((reaction) => (
        <div key={reaction.name} className="flex items-center mt-2">
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
  );
}

export default RenderReactions;
