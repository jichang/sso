import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ApplicationBasicComponent } from './application-basic.component';

describe('ApplicationBasicComponent', () => {
  let component: ApplicationBasicComponent;
  let fixture: ComponentFixture<ApplicationBasicComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ApplicationBasicComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ApplicationBasicComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should be created', () => {
    expect(component).toBeTruthy();
  });
});
