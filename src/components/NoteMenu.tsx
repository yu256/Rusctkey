import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import Modal from "react-modal";

Modal.setAppElement("#root");

const modalStyle = {
  overlay: {
    background: "rgba(0, 0, 0, 0.5)",
  },
  content: {
    overflow: "clip",
    height: "38dvh",
    bottom: "50dvh",
    border: "none",
    background: "white",
    borderRadius: "2em",
    width: "47dvh",
    margin: "auto",
  },
};

function NoteMenu() {
  const [isOpen, setIsOpen] = useState(false);
  let inputValue: string;

  function openModal() {
    setIsOpen(true);
  }

  function closeModal() {
    setIsOpen(false);
  }

  function handleInputChange(e: { target: { value: string } }) {
    inputValue = e.target.value;
  }

  async function post() {
    (await invoke("post", { text: inputValue }))
      ? closeModal()
      : console.log("投稿失敗");
  }

  function handleSubmit(e: { preventDefault: () => void }) {
    e.preventDefault();
    post();
  }

  return (
    <div className="block">
      <button
        onClick={openModal}
        className="rounded-full aspect-square fixed bottom-1 right-1"
      >
        <img src="/tabler-icons/pencil.svg" />
      </button>

      <Modal
        isOpen={isOpen}
        onRequestClose={closeModal}
        style={modalStyle}
        contentLabel="入力メニュー"
      >
        <form onSubmit={handleSubmit}>
          <button type="submit" className="float-right">
            <img src="/tabler-icons/send.svg" />
          </button>
          <textarea
            onChange={handleInputChange}
            autoFocus={true}
            className="w-[40dvh] h-[20dvh] border-pink-400 solid border-2 rounded-xl box-border outline-none resize-none"
          />
          {/* <div className="flex">
                        <button> TODO ファイルをアップロードしてモーダルの下に表示するやつ
                        </button>
                    </div> */}
        </form>
      </Modal>
    </div>
  );
}

export default NoteMenu;
