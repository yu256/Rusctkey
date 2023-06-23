import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import Modal from "react-modal";
import { DriveFile } from "../interfaces/drivefile";

Modal.setAppElement("#root");

const modalStyle = {
  overlay: {
    background: "rgba(0, 0, 0, 0.5)",
  },
  content: {
    border: "none",
    background: "white",
    borderRadius: "2em",
    width: "19em",
    margin: "auto",
    height: "25em",
  },
};

function NoteMenu() {
  const [isOpen, toggleIsOpen] = useState(false);
  const [files, setFiles] = useState<DriveFile[]>();
  let text: string;

  function toggleModal() {
    toggleIsOpen(!isOpen);
  }

  function input(e: React.ChangeEvent<HTMLTextAreaElement>) {
    text = e.target.value;
  }

  async function post() {
    if (await invoke("post", { text, files })) {
      toggleModal();
      setFiles(undefined);
    }
  }

  async function upload() {
    setFiles(await invoke("upload_files"));
  }

  return (
    <div className="block">
      <button
        onClick={toggleModal}
        className="rounded-full aspect-square fixed bottom-1 right-1"
      >
        <img src="/tabler-icons/pencil.svg" />
      </button>

      <Modal
        isOpen={isOpen}
        onRequestClose={toggleModal}
        style={modalStyle}
        contentLabel="入力メニュー"
      >
        <div className="flex">
          <button onClick={upload} className="w-16 h-16 rounded-full">
            <img src="/tabler-icons/photo-up.svg" />
          </button>
          <button onClick={post} className="w-16 h-16 rounded-full">
            <img src="/tabler-icons/send.svg" />
          </button>
        </div>
        <form>
          <textarea
            onChange={input}
            autoFocus={true}
            className="w-64 h-64 border-pink-400 solid border-2 rounded-xl box-border outline-none resize-none"
          />
        </form>
        {files && (
          <div className="flex">
            {files.map((file, index) => (
              <div
                key={index}
                className="m-1 relative w-16 h-9 bg-gray-500"
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
      </Modal>
    </div>
  );
}

export default NoteMenu;
