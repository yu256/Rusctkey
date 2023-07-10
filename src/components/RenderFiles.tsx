import { File } from "../interfaces/note";

interface Props {
  files: File[];
}

function RenderFiles({ files }: Props) {
  function parseFiles(file: File) {
    if (file.type.startsWith("image"))
      return (
        <div className="m-1 relative w-64 h-36 bg-gray-500">
          <img
            className="w-full h-full object-contain absolute"
            src={file.thumbnailUrl}
            alt={file.name}
          />
        </div>
      );
    if (file.type.startsWith("video"))
      return (
        <div className="m-1 w-64 h-36">
          <video src={file.url} controls />
        </div>
      );
    if (file.type.startsWith("audio"))
      return <audio src={file.url} controls />;
    return <>{file.name}</>;
  }
  return (
    <div className="flex flex-wrap">
      {files.map((file) => (
        <span key={file.id}>{parseFiles(file)}</span>
      ))}
    </div>
  );
}

export default RenderFiles;
