<template>
  <div class="hello">
    <p v-model="msg">
      {{msg}}
    </p>

    <textarea v-on:input="change" v-model="content">
      ## O M G !
      *tql*
    </textarea>
  </div>
</template>

<script>

const rust = import('../../../pkg');

export default {
  name: 'HelloWorld',
  props: {
    msg: String,
    content: {
      type: String,
      default: "{{who}}"
    }
  },
  methods: {
    change(s) {
      rust
          .then(module => {
            const ctx = {
              who: "<h1>OMG</h1>"
            }

            const result = module.render(ctx, this.content)
            this.msg = result
          })
          .catch(console.error);
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
