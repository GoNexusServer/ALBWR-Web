document.getElementById('fileInput').addEventListener('change', function() {
    document.getElementById('fileName').innerHTML = this.files?.length == 1 ? selectedFileHTML(this.files[0].name) : '';
    document.getElementById('uploadButtonChunk').disabled = this.files?.length < 1;
});

const dropArea = document.getElementById('dropArea');
const fileInput = document.getElementById('fileInput');

dropArea.addEventListener('dragover', handleDragOver);
dropArea.addEventListener('dragleave', handleDragLeave);
dropArea.addEventListener('drop', handleDrop);
dropArea.addEventListener('click', () => fileInput.click());

function handleDragOver(event) {
    event.preventDefault();
    dropArea.style.backgroundColor = '#f0f0f0';
}

function selectedFileHTML(fileName) {
    return `<p>Selected file:</p>` + `<p style="font-weight: 500 !important;">${fileName}</p>`
}

function handleDragLeave(event) {
    event.preventDefault();
    dropArea.style.backgroundColor = '';
}

function handleDrop(event) {
    event.preventDefault();
    dropArea.style.backgroundColor = '';
    const files = event.dataTransfer.files;
    document.getElementById('fileName').innerHTML = files?.length == 1 ? selectedFileHTML(files[0].name) : '';
    document.getElementById('uploadButtonChunk').disabled = files?.length < 1;
    if (files.length == 1) {
        fileInput.files = files; // Set files for the input element
        
        handleFileSelect({ target: { files: files } }); // Call handleFileSelect function with the files
    } else if (fileInput.files) delete fileInput.files;
}

function setBar(percent) {
    const progressBar = document.getElementById('progressBar');
    const progressBarInner = document.getElementById('progressBarInner');

    progressBar.style.display = 'block';

    if (percent === 101) {
        progressBar.classList.remove('progressBarClass');
        progressBar.innerHTML = `<p> Upload complete! Refreshing the page... </p>`;
        window.location.reload();
        return;
    }

    progressBarInner.style.width = `${percent}%`;
}
