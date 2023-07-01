import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
import ReactModal from 'react-modal';

function Widgets() {
  const [isOpen, toggleIsOpen] = useState(false);

  function toggleModal() {
    toggleIsOpen(!isOpen);
  }
  const modalStyle = {
	overlay: {
	  background: "rgba(0, 0, 0, 0.5)",
	},
	content: {
	  border: "none",
	  background: "white",
	  borderRadius: "2em",
	  margin: "auto",
	  height: "full",
	},
  };
  

  return (
    <div>
      <button
        onClick={toggleModal}
        className="aspect-square fixed bottom-1 left-1"
      >
        <img src="/tabler-icons/pencil.svg" />
      </button>

      <ReactModal
        isOpen={isOpen}
        onRequestClose={toggleModal}
        style={modalStyle}
        contentLabel="入力メニュー"
      >
		<button onClick={async () => await invoke("get_user") ? console.log("Yes") : console.log("No")}>
			クリック
		</button>
      </ReactModal>
	</div>
  );
}

export default Widgets;