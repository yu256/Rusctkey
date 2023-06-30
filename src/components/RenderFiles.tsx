import { Files } from "../interfaces/note";

interface Props {
  files: Files[];
}

function RenderFiles({ files }: Props) {
  return (
    <div className="flex flex-wrap">
      {files.map((file) => (
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
  );
}

export default RenderFiles;
