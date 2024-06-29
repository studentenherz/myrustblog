document.addEventListener("DOMContentLoaded", () => {
    const highlightCurrent = (entries) => {
        console.log(entries)

        entries.forEach(({isIntersecting, target}) => {
            if (isIntersecting) {
                document.getElementById(`ct-${target.id}`).classList.add("current")
            }
            else {
                document.getElementById(`ct-${target.id}`).classList.remove("current")
            }
        })
    }

    let observer = new IntersectionObserver(highlightCurrent, {threshold: 0.3})

    document.querySelectorAll(".post section").forEach(heading => {
        observer.observe(heading)
    })
})
