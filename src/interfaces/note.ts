export interface Note {
  id: String;
  createdAt: string;
  modifiedCreatedAt: string;
  user: User;
  text?: string;
  reactions: Map<string, number>;
  emojis: Emoji[];
  files: Files[];
  renote?: Renote;
}

interface Renote {
  id: String;
  createdAt: string;
  modifiedCreatedAt: string;
  user: User;
  text?: string;
  reactions: Map<string, number>;
  emojis: Emoji[];
  files: Files[];
}

interface Files {
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

interface Emoji {
  name: string;
  url: string;
}

interface User {
  username: string;
  host?: string;
  name: string;
  avatarUrl: string;
  instance: Instance;
  onlineStatus: string;
  emojis: Emoji[];
}

interface Instance {
  name: string;
  softwareName: string;
  softwareVersion: string;
  iconUrl: string;
  faviconUrl: string;
  themeColor: string;
}
