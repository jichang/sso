import { Component, OnInit } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import { HttpErrorResponse } from "@angular/common/http";
import { MatSnackBar } from "@angular/material/snack-bar";

@Component({
  selector: "app-scope-create-page",
  templateUrl: "./scope-create-page.component.html",
  styleUrls: ["./scope-create-page.component.css"]
})
export class ScopeCreatePageComponent implements OnInit {
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit() {}

  created(evt) {
    this.router.navigate(["../.."], { relativeTo: this.route });
  }

  failure(err: HttpErrorResponse) {
    if (err.status === 409) {
      this.snackBar.open("Email used", "Dismiss", {
        duration: 3000
      });
    } else {
      this.snackBar.open("Unknown error", "Dismiss", {
        duration: 3000
      });
    }
  }
}
