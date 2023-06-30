export interface Note {
  id: string;
  createdAt: string;
  user: User;
  text?: string;
  modifiedEmojis?: Reactions;
  files: Files[];
  renote?: Renote;
}

interface Reactions {
  reactions: Reaction[];
}

export interface Reaction {
  name: string;
  url: string;
  count: number;
}

export interface Renote {
  id: string;
  createdAt: string;
  user: User;
  text?: string;
  modifiedEmojis?: Reactions;
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

interface User {
  username: string;
  host?: string;
  name: string;
  avatarUrl: string;
  instance: Instance;
  onlineStatus: string;
  emojis: Map<string, string>;
}

interface Instance {
  name?: string;
  softwareName?: string;
  softwareVersion?: string;
  iconUrl?: string;
  faviconUrl?: string;
  themeColor?: string;
}
