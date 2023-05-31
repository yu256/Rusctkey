export interface DriveFile {
  id: string;
  createdAt: string;
  name?: string;
  type: string;
  md5: string;
  size: number;
  isSensitive: boolean;
  blurhash?: string;
  properties: Properties;
  url: string;
  thumbnailUrl?: string;
  comment?: string;
  folderId?: string;
  folder?: string;
  userId?: string;
  user?: string;
}

interface Properties {
  width?: number;
  height?: number;
}
