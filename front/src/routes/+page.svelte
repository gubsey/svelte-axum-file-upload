<script lang="ts">
    import { onMount } from "svelte";

    let files: FileList | null;
    let future: Promise<Response> | null;

    let file_data: string | undefined;

    const readFile = (file: File): Promise<string> => {
        return new Promise((resolve) => {
            const reader = new FileReader();
            reader.onload = () => resolve(reader.result as string);
            reader.readAsDataURL(file);
        });
    };

    const handleFileUpload = async (event: SubmitEvent) => {
        const form_data = new FormData(event.target as HTMLFormElement);
        const file = form_data.get("file") as File;

        alert(file.name);

        file_data = await readFile(file);

        let body = JSON.stringify({ file_data });
        console.log(body);

        const res = await fetch("http://localhost:1971", {
            method: "POST",
            body,
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!res.ok) {
            const error = await res.json();
            alert(error.message);
        }

        file_data = undefined;
        (event.target as HTMLFormElement).reset();

        const data = await res.json();

        alert(data);
    };

    onMount(async () => {
        let res = await fetch("http://localhost:1971/tyler");
        console.log(res.status);
        tyler = await res.text();
    });
</script>

<form on:submit|preventDefault={handleFileUpload}>
    <div class="group">
        <label for="file">Upload yout profile picture</label>
        <input type="file" id="file" name="file" accept=".txt" required />
    </div>

    <button type="submit">Submit</button>
</form>
