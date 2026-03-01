export default function Home() {
  return (
    <div className="p-8 mx-auto max-w-[400px] text-center">
      <h1 className="text-2xl font-bold mb-5 mt-12">Notes</h1>
      <div>
        <button className="px-4 py-2 bg-oni-violet text-white rounded-lg">
          Create Note
        </button>
      </div>
    </div>
  );
}
