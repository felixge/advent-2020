package main

import "testing"

func TestValid(t *testing.T) {
	tests := []struct {
		entry entry
		want  bool
	}{
		{
			entry: entry{
				policy:   policy{1, 3, 'a'},
				password: "abc",
			},
			want: true,
		},
		{
			entry: entry{
				policy:   policy{1, 3, 'a'},
				password: "aac",
			},
			want: true,
		},
		{
			entry: entry{
				policy:   policy{1, 3, 'a'},
				password: "aaa",
			},
			want: true,
		},
		{
			entry: entry{
				policy:   policy{1, 3, 'a'},
				password: "aaab",
			},
			want: true,
		},
		{
			entry: entry{
				policy:   policy{1, 3, 'a'},
				password: "bbb",
			},
			want: false,
		},
	}

	for _, test := range tests {
		got := test.entry.Valid()
		if got != test.want {
			t.Fatalf("got=%t want=%t: %#v", got, test.want, test.entry)
		}
	}
}
