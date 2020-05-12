function makeRequest(state) {
  $.post("/fireplace", {fireplaceState: state})
    .done(function() {
      console.log("successfully set fireplace status");
    })
    .fail(function() {
      console.log("failed to set fireplace status");
    });
}


$("#allOn").click(function(e) {
  e.preventDefault();
  makeRequest("allOn")
});

$("#fireplaceOn").click(function(e) {
  e.preventDefault();
  makeRequest("fireplaceOn")
});

$("#allOff").click(function(e) {
  e.preventDefault();
  makeRequest("allOff")
});
