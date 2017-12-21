import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ApplicationCreatePageComponent } from './application-create-page.component';

describe('ApplicationCreatePageComponent', () => {
  let component: ApplicationCreatePageComponent;
  let fixture: ComponentFixture<ApplicationCreatePageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ApplicationCreatePageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ApplicationCreatePageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should be created', () => {
    expect(component).toBeTruthy();
  });
});
